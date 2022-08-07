package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/awslabs/aws-lambda-go-api-proxy/gin"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

var tableName = os.Getenv("TABLE_NAME")
var dynamo *dynamodb.Client
var partitionKey string
var apiKey string
var logger zap.SugaredLogger
var ginLambda *ginadapter.GinLambda

func isInLambda() bool {
	if os.Getenv("AWS_EXECUTION_ENV") == "AWS_Lambda_go1.x" {
		return true
	}
	return false
}

func Authenticate(ctx *gin.Context) {
	apiKey = ctx.Request.Header.Get("x-api-key")
	if apiKey != "" {
		partitionKey = fmt.Sprintf("RULE#%s", apiKey)
		ctx.Next()
	} else {
		ctx.AbortWithStatus(http.StatusUnauthorized)
	}
}

func router() *gin.Engine {
	r := gin.Default()
	r.Use(Authenticate)
	r.GET("/dev/rule/:name", GetRule)
	r.PUT("/dev/rule/:name", CreateRule)
	r.GET("/dev/rules", ListRules)

	return r
}

func init() {
	var z *zap.Logger
	var err error

	if stage := os.Getenv("STAGE"); stage != "prod" {
		z, err = zap.NewDevelopment()
	} else {
		z, err = zap.NewProduction()
	}

	if err != nil {
		log.Fatal(err)
	}

	cfg, err := config.LoadDefaultConfig(context.TODO())

	if err != nil {
		logger.Errorf("Unable to load SDK config %v", err)
	}

	dynamo = dynamodb.NewFromConfig(cfg)

	logger = *z.Sugar()
	logger.Info("Cold Start")

	r := router()
	if isInLambda() {
		ginLambda = ginadapter.New(r)
	} else {
		http.ListenAndServe(":8080", r)
	}
}

func Handler(ctx context.Context, req events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	return ginLambda.ProxyWithContext(ctx, req)
}

func main() {
	if isInLambda() {
		lambda.Start(Handler)
	}
}
