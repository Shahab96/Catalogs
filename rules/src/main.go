package main

import (
	"context"
	"log"
	"net/http"
	"os"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go/aws/session"
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

var logger zap.SugaredLogger
var ginLambda *ginadapter.GinLambda

func isInLambda() bool {
	return os.Getenv("AWS_EXECUTION_ENV") == "AWS_LAMBDA_go1.x"
}

func router() *gin.Engine {
	r := gin.Default()
	r.GET("/rule/:name", GetRule)

	return r
}

func init() {
	z, err := zap.NewProduction()

	if err != nil {
		log.Fatal(err)
	}

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
