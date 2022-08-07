package main

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
)

func GetRule(ctx *gin.Context) {
	name := ctx.Param("name")

	rule, status := GetRuleDynamo(name, apiKey)

	switch status {
	case 200:
		ctx.IndentedJSON(http.StatusOK, rule)

	case 404:
		ctx.Status(http.StatusNotFound)
		ctx.Error(fmt.Errorf("Rule named %s was not found.", name))

	case 500:
		ctx.Status(http.StatusInternalServerError)
		ctx.Error(fmt.Errorf("There was an error processing your request."))
	}
}

func CreateRule(ctx *gin.Context) {
	var rule Rule

	if err := ctx.BindJSON(&rule); err != nil {
		ctx.Status(http.StatusBadRequest)
		ctx.Error(fmt.Errorf("Invalid Rule."))
		return
	}

	status := PutRuleDynamo(&rule, apiKey)

	switch status {
	case 201:
		ctx.IndentedJSON(http.StatusCreated, rule)

	case 500:
		ctx.Status(http.StatusInternalServerError)
		ctx.Error(fmt.Errorf("There was an error processing your request."))
	}
}
