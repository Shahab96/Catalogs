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
	"github.com/bahadirbb/zapcloudwatch"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
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

func Authenticate(ctx *gin.Context) {
	if _, ok := ctx.Request.Header["x-api-key"]; ok {
		logger.Info("Authenticated")
		ctx.Next()
	} else {
		ctx.AbortWithStatus(http.StatusUnauthorized)
	}
}

func router() *gin.Engine {
	r := gin.Default()
	r.Use(Authenticate)
	r.GET("/rule/:name", GetRule)

	return r
}

func init() {
	logGroup := os.Getenv("AWS_LAMBDA_LOG_GROUP_NAME")
	logStream := os.Getenv("AWS_LAMBDA_LOG_STREAM_NAME")
	cloudwatchHook, err := zapcloudwatch.NewCloudwatchHook(logGroup, logStream, false, sess.Config, zapcore.InfoLevel).GetHook()
	if err != nil {
		panic(err)
	}

	config := zap.NewProductionConfig()
	config.Encoding = "json"
	z, err := config.Build()
	z = z.WithOptions(zap.Hooks(cloudwatchHook)).Named("Logger")

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
		logger.Debug("Running in Lambda")
		lambda.Start(Handler)
	} else {
		logger.Debug("Running locally")
	}
}
