REPO="scistarter/circuit-frontend"
RELEASE=0.1
PATCH=`cat PATCH`

version:
	@echo "$(RELEASE).$(PATCH)"

imageid:
	@echo "$(REPO):$(RELEASE).$(PATCH)"

bump:
	echo $$(( $(PATCH) + 1 )) > PATCH

image: bump ../Dockerfile.frontend
	P=$(shell cat PATCH)
	cd .. && docker build -f Dockerfile.frontend -t "$(REPO):$(RELEASE).$(P)" .

upload: image
	docker push "$(REPO):$(RELEASE).$(PATCH)"
	@echo "Uploaded $(RELEASE).$(PATCH)"

.PHONY: image upload bump version
