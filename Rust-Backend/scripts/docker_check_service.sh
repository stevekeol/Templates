#################################################
#
# Check all dependencies-services is up.
#
#################################################


#!/usr/bin/env bash

source ./scripts/style_info.cfg

docker_compose_components=(
  mongodb
  # mysql
  # open-im-server
  # redis
  # kafka
  # zookeeper
)

component_server_count=0

for ((i = 0; i < ${#docker_compose_components[*]}; i++)); do
  component_server="docker-compose ps|grep -w ${docker_compose_components[$i]}|grep Up"
  count="${component_server}|wc -l"

  if [ $(eval ${count}) -gt 0 ]; then
    echo -e "✅ ${SKY_BLUE_PREFIX}docker-compose ${docker_compose_components[$i]} is Up!${COLOR_SUFFIX}"
    let component_server_count+=1
  else
    echo -e "❌ ${RED_PREFIX} ${docker_compose_components[$i]} NOT up!${COLOR_SUFFIX}"
    echo -e "🏗️ ${YELLOW_PREFIX}Attempt to start the ${docker_compose_components[$i]} container...${COLOR_SUFFIX}"
    sudo docker-compose start -d mongodb; # replace up(会先构建（如果需要）然后启动服务) -> start(直接启动已经构建好的服务容器)
  fi
done

# -eq 1 需随上述依赖组件的个数而调整(目前就一个依赖，索性暂时不显示了)
# if [ ${component_server_count} -eq 1 ]; then
#   echo -e "✌️ ${YELLOW_PREFIX}\ndocker-compose all services is Up!${COLOR_SUFFIX}"
# else
#   echo -e "🔴 ${RED_PREFIX}\nsome docker-compose services start failed, please check red logs on console ${COLOR_SUFFIX}"
# fi
