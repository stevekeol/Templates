# Code Arch

## The Complete Project Folder Structure

```yaml title="源文件结构及功能"
.
├── Dockerfile: Docker配置
├── docker-compose.yaml: Docker容器编排
├── api: [外界统一的请求接口api]
│   ├── controller: [在route的时候，将每个request dispatch到对应的controller上]
│   │   ├── login_controller.go: 
│   │   ├── profile_controller.go: 
│   │   ├── profile_controller_test.go: 
│   │   ├── refresh_token_controller.go: 
│   │   ├── signup_controller.go:  
│   │   └── task_controller.go: 
│   ├── middleware: [在route(组)通过挂载middleware来统一做某些事]
│   │   └── jwt_auth_middleware.go: jwt授权中间件
│   └── route: [将Port上监听的request路由到具体handler中]
│       ├── login_route.go:  
│       ├── profile_route.go: 
│       ├── refresh_token_route.go:  
│       ├── route.go: 所有route的统一配置入口
│       ├── signup_route.go: 
│       └── task_route.go: 
├── bootstrap: [server的启动项]
│   ├── app.go: 组装app（往app上挂载env和database）
│   ├── database.go: 构建一个database(mongodb)的client
│   └── env.go: 构建一个env实例(读取env文件中的配置)
├── cmd: 
│   └── main.go: 真正运行server App的地方(也是监听request的开始)
├── domain: 
│   ├── error_response.go: 
│   ├── jwt_custom.go: 
│   ├── login.go: 
│   ├── profile.go: 
│   ├── refresh_token.go: 
│   ├── signup.go: 
│   ├── success_response.go: 
│   ├── task.go: 
│   └── user.go: 
├── internal: 
│   └── tokenutil: 
│       └── tokenutil.go: 
├── mongo: 
│   └── mongo.go: 
├── repository: 
│   ├── task_repository.go: 
│   ├── user_repository.go: 
│   └── user_repository_test.go: 
└── usecase: [独立于具体的框架、库或技术实现，实现业务逻辑]
    ├── login_usecase.go: 
    ├── profile_usecase.go: 
    ├── refresh_token_usecase.go: 
    ├── signup_usecase.go: 
    ├── task_usecase.go: 
    └── task_usecase_test.go: 
```

## 详解
1. usecase
> 可通过创建一个结构体来定义usecase，然后在其中编写业务逻辑代码。使用依赖注入模式，将所需依赖项注入其中，以便和其它层进行交互


## 升级(或并列)为COLA架构

```yaml title="源文件对应的迁移位置"
.
├── Dockerfile: 
├── docker-compose.yaml: 
├── api: api(先统一放在api中，后续再视情况拆解到)
│   ├── controller: [在route的时候，将每个request dispatch到对应的controller上]
│   │   ├── login_controller.go: 
│   │   ├── profile_controller.go: 
│   │   ├── profile_controller_test.go: 
│   │   ├── refresh_token_controller.go: 
│   │   ├── signup_controller.go:  
│   │   └── task_controller.go: 
│   ├── middleware: [在route(组)通过挂载middleware来统一做某些事]
│   │   └── jwt_auth_middleware.go: jwt授权中间件
│   └── route: [将Port上监听的request路由到具体handler中]
│       ├── login_route.go:  
│       ├── profile_route.go: 
│       ├── refresh_token_route.go:  
│       ├── route.go: 所有route的统一配置入口
│       ├── signup_route.go: 
│       └── task_route.go: 
├── bootstrap: infra/cmd/bootstrap
│   ├── app.go: 
│   ├── database.go: (启动database)
│   └── env.go: (读取env配置)
├── cmd: infra/cmd
│   └── main.go: (启动)
├── domain: domain
│   ├── error_response.go: 
│   ├── jwt_custom.go: 
│   ├── login.go: 
│   ├── profile.go: 
│   ├── refresh_token.go: 
│   ├── signup.go: 
│   ├── success_response.go: 
│   ├── task.go: 
│   └── user.go: 
├── internal: infra/internal
│   └── tokenutil: 
│       └── tokenutil.go: 
├── mongo: infra/database/mongodb/mongo.go
│   └── mongo.go: (跟业务无关的Mongo操作封装层)
├── repository: infra/database/mongodb/xxx.go (业务操作封装层——主要是Domain中的Model，注意有些Model并不需要repository，如login/signup等)
│   ├── task_repository.go: 
│   ├── user_repository.go: 
│   └── user_repository_test.go: 
└── usecase: infra/usecase (Infra中实现Domain中Gateway约定的接口)
    ├── login_usecase.go: 
    ├── profile_usecase.go: 
    ├── refresh_token_usecase.go: 
    ├── signup_usecase.go: 
    ├── task_usecase.go: 
    └── task_usecase_test.go: 
```