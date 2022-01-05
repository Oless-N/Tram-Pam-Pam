<<<<<<< HEAD
CODE_DIRS=game/ tests/
PYTEST=pytest -v -s tests --log-level=INFO
COV_TYPE ?=html
DOCKER_PROJECT=buildozer
APP_CONTAINER=buildozer
DOCKER_RUN=docker run -it --rm -v `pwd`:/game
DOCKER_BUILD=docker build -t ${APP_CONTAINER}
DOCKER_LOCAL=docker-compose -f docker-compose.yml
COLS_LINES=-e COLUMNS="`tput cols`" -e LINES="`tput lines`"


clear:
	docker ps -a --format "{{ .ID }}" | xargs docker rm -f
	docker image ls -a --format "{{ .ID }}" | xargs docker rmi -f

stop:
	printf "=========Stop app ${IMAGE}=========\n\n"
	$(DOCKER_LOCAL) stop

rm: stop
	printf "=========Remove app ${IMAGE}=========\n\n"
	$(DOCKER_LOCAL) rm -f

up:
	printf "=========Start app ${IMAGE}=========\n\n"
	$(DOCKER_LOCAL) up -d

down:
	printf "=========Down app ${IMAGE}=========\n\n"
	$(DOCKER_LOCAL) down

ps:
	printf "=========Down app ${IMAGE}=========\n\n"
	$(DOCKER_LOCAL) ps

build:
	printf "=========Build image ${IMAGE}=========\n\n"
	$(DOCKER_LOCAL) build

restart:
	$(DOCKER_LOCAL) stop
	$(DOCKER_LOCAL) down --rmi local --remove-orphans
	$(DOCKER_LOCAL) build
	$(DOCKER_LOCAL) up -d

attach:
	docker attach buildozer

status:
	$(DOCKER_LOCAL) ps


bash:
	$(DOCKER_LOCAL) exec $(COLS_LINES) buildozer /bin/bash

logs:
	COMPOSE_HTTP_TIMEOUT=604800 $(DOCKER_LOCAL) logs -f

log:
	COMPOSE_HTTP_TIMEOUT=604800 docker-compose logs -f ${name}

flake8:
	$(DOCKER_RUN) $(APP_CONTAINER) flake8 --config setup.cfg $(CODE_DIRS)

run: 
	python3 game/main.py

install_dev:
	pip install -U pip setuptools wheel cython gitpython
	pip install -r requirements/base.txt

.PHONY: shell
=======
app:
	doxker-compose up
>>>>>>> 2bc84e320982af9510603982796afeef0ac82f1f
