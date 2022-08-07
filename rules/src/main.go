package main

import (
	"context"
	"log"
	"net/http"
	"os"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"github.com/aws/aws-xray-sdk-go/xray"
	"github.com/awslabs/aws-lambda-go-api-proxy/gin"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

var sess = session.Must(
	session.NewSessionWithOptions(
		session.Options{
			SharedConfigState: session.SharedConfigEnable,
		},
	),
)

var requestContext context.Context
var dynamo *dynamodb.DynamoDB
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

	xray.AWSSession(sess)
	dynamo = dynamodb.New(sess)

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
	requestContext = ctx
	return ginLambda.ProxyWithContext(ctx, req)
}

func main() {
	if isInLambda() {
		lambda.Start(Handler)
	}
}
