package main

import (
	"time"

	route "Taiki-Server-Go/api/route"
	"Taiki-Server-Go/bootstrap"
	"github.com/gin-gonic/gin"
)

func main() {
	// 1.获取整个App实例
	app := bootstrap.App()
	// 2.获取App实例中的env字段
	env := app.Env
	// 3.获取App实例中的db(逐层获取)
	db := app.Mongo.Database(env.DBName)
	defer app.CloseDBConnection()

	gin := gin.Default()
	// 4.执行Setup(用于挂载路由)
	// TODO: 传入env等这些参数，似乎可以用context来实现，不然每一个route都需要写一次
	route.Setup(env, time.Duration(env.ContextTimeout)*time.Second, db, gin)
	// 5.运行
	gin.Run(env.ServerAddress)
}
