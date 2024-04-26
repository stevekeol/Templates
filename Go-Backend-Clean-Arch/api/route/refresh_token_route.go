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

func NewRefreshTokenRouter(env *bootstrap.Env, timeout time.Duration, db mongo.Database, group *gin.RouterGroup) {
	ur := repository.NewUserRepository(db, domain.CollectionUser)
	rtc := &controller.RefreshTokenController{
		RefreshTokenUsecase: usecase.NewRefreshTokenUsecase(ur, timeout),
		Env:                 env,
	}
	group.POST("/refresh", rtc.RefreshToken)
}
