REPO:="scistarter/circuit-api-`git branch --show-current`"
RELEASE:=0.1
PATCH=`cat PATCH`

version:
	@echo "$(RELEASE).$(PATCH)"

imageid:
	@echo "$(REPO):$(RELEASE).$(PATCH)"

bump:
	echo $$(( $(PATCH) + 1 )) > PATCH

image: bump ../Dockerfile.api
	docker build -f ../Dockerfile.api -t "$(REPO):$(RELEASE).$(PATCH)" ..

upload: image
	docker push "$(REPO):$(RELEASE).$(PATCH)"
	@echo "Uploaded $(RELEASE).$(PATCH)"

.PHONY: image upload bump version
