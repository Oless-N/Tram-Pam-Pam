flake:
	$(DOCKER_RUN) $(APP_CONTAINER) flake8 --config setup.cfg $(CODE_DIRS)

run: 
	python game/main.py

install_dev:
	pip install -U pip setuptools wheel cython gitpython
	pip install -r requirements/base.txt

.PHONY: shell

