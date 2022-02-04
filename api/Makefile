REPO:="scistarter/circuit-api"
RELEASE:=0.1
PATCH=`cat PATCH`
BRANCH=`git branch --show-current`

version:
	@echo "$(RELEASE).$(PATCH)"

imageid:
	@echo "$(REPO):$(BRANCH)-$(RELEASE).$(PATCH)"

bump:
	echo $$(( $(PATCH) + 1 )) > PATCH

image: bump ../Dockerfile.api
	docker build -f ../Dockerfile.api -t "$(REPO):$(BRANCH)-$(RELEASE).$(PATCH)" ..

upload: image
	docker push "$(REPO):$(BRANCH)-$(RELEASE).$(PATCH)"
	@echo "Uploaded $(BRANCH)-$(RELEASE).$(PATCH)"

.PHONY: image upload bump version
