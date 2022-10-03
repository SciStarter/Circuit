REPO:="scistarter/circuit-uploader"
RELEASE:=0.1
PATCH=`cat PATCH`

version:
	@echo "$(RELEASE).$(PATCH)"

imageid:
	@echo "$(REPO):$(RELEASE).$(PATCH)"

bump:
	echo $$(( $(PATCH) + 1 )) > PATCH

image: bump ../../Dockerfile.uploader
	docker build -f ../../Dockerfile.uploader -t "$(REPO):$(RELEASE).$(PATCH)" ../..

upload: image
	docker push "$(REPO):$(RELEASE).$(PATCH)"
	@echo "Uploaded $(RELEASE).$(PATCH)"

.PHONY: image upload bump version
