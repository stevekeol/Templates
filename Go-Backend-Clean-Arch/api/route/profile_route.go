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

func NewProfileRouter(env *bootstrap.Env, timeout time.Duration, db mongo.Database, group *gin.RouterGroup) {
	ur := repository.NewUserRepository(db, domain.CollectionUser)
	pc := &controller.ProfileController{
		ProfileUsecase: usecase.NewProfileUsecase(ur, timeout),
	}
	group.GET("/profile", pc.Fetch)
}
