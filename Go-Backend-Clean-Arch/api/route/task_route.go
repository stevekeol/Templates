package route

import (
	"time"

	"Taiki-Server-Go/api/controller"
	"Taiki-Server-Go/bootstrap"
	"Taiki-Server-Go/domain"
	"Taiki-Server-Go/mongo"
	"Taiki-Server-Go/repository"
	"Taiki-Server-Go/usecase"
	"github.com/gin-gonic/gin"
)

func NewTaskRouter(env *bootstrap.Env, timeout time.Duration, db mongo.Database, group *gin.RouterGroup) {
	tr := repository.NewTaskRepository(db, domain.CollectionTask)
	tc := &controller.TaskController{
		TaskUsecase: usecase.NewTaskUsecase(tr, timeout),
	}
	group.GET("/task", tc.Fetch)
	group.POST("/task", tc.Create)
}
