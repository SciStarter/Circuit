frontend: frontend/Dockerfile
	docker build -t circuit-frontend frontend

backend: backend/Dockerfile
	docker build -t circuit-backend backend

.PHONY: frontend backend
