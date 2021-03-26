RELEASE=0.1
PATCH:=$(shell cat PATCH)

frontend: frontend/Dockerfile
	docker build -t "scistarter/circuit-frontend:$(RELEASE).$(PATCH)" frontend

backend: backend/Dockerfile
	docker build -t "scistarter/circuit-backend:$(RELEASE).$(PATCH)" backend

upload: frontend backend
	docker push "scistarter/circuit-frontend:$(RELEASE).$(PATCH)"
	docker push "scistarter/circuit-backend:$(RELEASE).$(PATCH)"
	echo "Uploaded $(RELEASE).$(PATCH)"
	echo $$(( $(PATCH) + 1 )) > PATCH

.PHONY: frontend backend
