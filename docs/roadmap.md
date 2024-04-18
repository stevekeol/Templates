# roadmap

> 所依赖的mongodb（以及以后可能依赖的redis, kafka, etcd等）均通过docker来运行

0. [x] 解耦production和development的数据库(db, db_test)

1. [x] 依赖组件的脚本前置运行，以及判断是否成功运行： （当没启动mongodb时，会自动借助docker启动）

2. [] api集成测试

3. [] 调整日志的生成位置

4. [] 授权路径

5. [] subTemplates

>

```yaml title="理想的启动方式（目前只需要更改.env文件中的cargo_env: develeopment/production）"
- .env文件应该拆分成两个（妥协!!!，因为默认读取的文件就是.env）: .env 和 .env.test （运行的端口和数据库名字有区别，前者用于可以在production模式下运行的同时，继续开发测试；后者用于区分测试数据和正式数据）
- 运行之前必须判断mongo等依赖组件是否已经成功启动
```

> 新环境下，是不是还得先装 docker 啊??? (将脚本仍旧放在 makefile中， make init)

makefile: test, run, run dev(TODO: 现在需要手动修改 .env文件), stop(stop app -> stop mongo)

> 执行命令需要 sudo ，解决办法: 写在shell中，赋予其权限即可。在执行命令的过程中，会暂停下来，让其输入密码。

