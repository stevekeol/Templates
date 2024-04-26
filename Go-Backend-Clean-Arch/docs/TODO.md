# TODO

> 零散的待增加的小细节

1. remove/exchange the non-offical package

	- `amitshekhariitbhu`相关的全部移植成官方的，但需要对官方的依赖进行版本锁定

2. 将项目内的依赖包，替换为相对路径而非绝对的github路径

3. middleware中 `rate limit` 的实现

4. route等中增加context，来将通用配置注入上下文context

5. 引入 code-arch-v2 (遥远计划)

6. 配置文件由 .env -> config.yaml，当然包括解析和配置的代码（参考OpenIM）

7. Makefile + Shell + Docker Compose
