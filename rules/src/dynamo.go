package main

import (
	"fmt"
	"os"

	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"github.com/aws/aws-sdk-go/service/dynamodb/dynamodbattribute"
)

type Rule struct {
	Name  string `json:"name"`
	Rule  string `json:"rule"`
	Owner string `json:"owner"`
}

func GetRuleDynamo(name string, apiKey string) (*Rule, int) {
	dynamo := dynamodb.New(sess)
	tableName := os.Getenv("TABLE_NAME")

	result, err := dynamo.GetItem(&dynamodb.GetItemInput{
		TableName: aws.String(tableName),
		Key: map[string]*dynamodb.AttributeValue{
			"id": {
				S: aws.String(fmt.Sprintf("RULE#%s", apiKey)),
			},
			"sort_key": {
				S: aws.String(name),
			},
		},
		ProjectionExpression: aws.String("name, rule"),
	})

	if err != nil {
		logger.Errorf("DynamoDB Error on GetItem: %s", err)
		return nil, 500
	}

	if result.Item == nil {
		logger.Errorf("Rule named %s not found", name)
		return nil, 404
	}

	rule := Rule{}

	err = dynamodbattribute.UnmarshalMap(result.Item, &rule)

	if err != nil {
		logger.Errorf("Failed to unmarshall record: %v", err)
		return nil, 500
	}

	return &rule, 200
}
