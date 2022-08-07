package main

import (
	"context"
	"fmt"
	"os"

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
	tableName := os.Getenv("TABLE_NAME")

	logger.Debugf("Attempt to fetch rule RULE#%s", apiKey)

	getItemInput := &dynamodb.GetItemInput{
		TableName: &tableName,
		Key: map[string]types.AttributeValue{
			"id":       &types.AttributeValueMemberS{Value: fmt.Sprintf("RULE#%s", apiKey)},
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
	tableName := os.Getenv("TABLE_NAME")

	logger.Debugf("Attempt to create RULE#%s %s %s", apiKey, rule.Name, rule.Rule)

	putItemInput := dynamodb.PutItemInput{
		TableName: &tableName,
		Item: map[string]types.AttributeValue{
			"id":       &types.AttributeValueMemberS{Value: fmt.Sprintf("RULE#%s", apiKey)},
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
