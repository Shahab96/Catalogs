package main

import (
	"context"
	"fmt"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/feature/dynamodb/attributevalue"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
)

type Rule struct {
	Name string `json:"name"`
	Rule string `json:"rule"`
}

func GetRuleDynamo(name string, apiKey string) (*Rule, int) {
	logger.Debugf("Attempt to fetch rule %s", partitionKey)

	getItemInput := &dynamodb.GetItemInput{
		TableName: &tableName,
		Key: map[string]types.AttributeValue{
			"id":       &types.AttributeValueMemberS{Value: fmt.Sprintf(partitionKey)},
			"sort_key": &types.AttributeValueMemberS{Value: name},
		},
		ExpressionAttributeNames: map[string]string{
			"#n": "name",
			"#r": "rule",
		},
		ProjectionExpression: aws.String("#n, #r"),
	}

	result, err := dynamo.GetItem(context.TODO(), getItemInput)

	if err != nil {
		logger.Errorf("DynamoDB Error on GetItem: %s", err)
		return nil, 500
	}

	if result.Item == nil {
		logger.Errorf("Rule named %s not found", name)
		return nil, 404
	}

	rule := Rule{}

	err = attributevalue.UnmarshalMap(result.Item, &rule)

	if err != nil {
		logger.Errorf("Failed to unmarshall record: %v", err)
		return nil, 500
	}

	return &rule, 200
}

func PutRuleDynamo(rule *Rule, apiKey string) int {
	logger.Debugf("Attempt to create %s %s %s", partitionKey, rule.Name, rule.Rule)

	putItemInput := dynamodb.PutItemInput{
		TableName: &tableName,
		Item: map[string]types.AttributeValue{
			"id":       &types.AttributeValueMemberS{Value: partitionKey},
			"sort_key": &types.AttributeValueMemberS{Value: rule.Name},
			"name":     &types.AttributeValueMemberS{Value: rule.Name},
			"rule":     &types.AttributeValueMemberS{Value: rule.Rule},
			"owner":    &types.AttributeValueMemberS{Value: apiKey},
		},
	}

	_, err := dynamo.PutItem(context.TODO(), &putItemInput)

	if err != nil {
		logger.Errorf("Error on DynamoDB PutItem %v", err)
		return 500
	}

	logger.Info("Rule created.")
	logger.Debugf("Rule %s created: %s for user %s", rule.Name, rule.Rule, apiKey)

	return 201
}

func ListRulesDynamo() (*[]Rule, int) {
	partitionKey := fmt.Sprintf("RULE#%s", apiKey)
	logger.Debugf("Attempt to fetch rules %s", partitionKey)

	listItemsQuery := dynamodb.QueryInput{
		TableName:              &tableName,
		KeyConditionExpression: aws.String("#id = :id"),
		ProjectionExpression:   aws.String("#n, #r"),
		ExpressionAttributeNames: map[string]string{
			"#id": "id",
			"#n":  "name",
			"#r":  "rule",
		},
		ExpressionAttributeValues: map[string]types.AttributeValue{
			":id": &types.AttributeValueMemberS{Value: partitionKey},
		},
	}

	paginator := dynamodb.NewQueryPaginator(dynamo, &listItemsQuery)

	var rules []Rule

	for paginator.HasMorePages() {
		page, err := paginator.NextPage(context.TODO())

		if err != nil {
			logger.Errorf("There was an error while fetching rules %s, %v", partitionKey, err)
			return nil, 500
		}

		err = attributevalue.UnmarshalListOfMaps(page.Items, &rules)

		if err != nil {
			logger.Errorf("There was an exception while unmarshalling response %v", err)
			return nil, 500
		}
	}

	return &rules, 200
}
