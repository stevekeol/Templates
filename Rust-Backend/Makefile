#!/usr/bin/env bash

# Taiki: clean build (combile multi commands)


init: 
	@echo "🌃 \033[36mInstall the docker on a new machine...\033[36m" # TODO: install docker by xx.sh

build:
	@cargo build

run: check
	@cargo run

clean:
	@echo "🗑️ \033[36mCleaning the target...\033[36m"
	@cargo clean # TODO: Clean database

check:
	@echo "🩺 \033[36mChecking the mongodb...\033[36m"
	@sudo bash scripts/docker_check_service.sh

# Just mongodb now
stop:
	@echo "🚨 \033[36mStopping the mongodb...\033[36m"
	@sudo docker-compose stop # 此处不能用down(注意两者区别)

# API Test
api-test:
	@echo "🧪 \033[36mTesting the API (with hurl)...\033[36m"
	@hurl test/api.hurl
