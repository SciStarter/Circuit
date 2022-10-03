<template>
<div class="upload-file">
  <form ref="file_picker">
    <input type="file" name="file">
  </form>
  <button type="button" @click="upload">Upload</button>
</div>
</template>

<script>
export default {
    name: "UploadFile",

    methods: {
        async upload() {
            let spinner = this.$buefy.loading.open();

            try {
                let uploaded = await this.$axios.$post("/api/upload", new FormData(this.$refs.file_picker));
            }
            catch(err) {
                spinner.close();
                this.$buefy.dialog.alert({
                    title: 'Error',
                    message: "Unable to communicate with the server",
                    type: 'is-danger',
                    hasIcon: true,
                    icon: 'times-circle',
                    iconPack: 'fa',
                    ariaRole: 'alertdialog',
                    ariaModal: true
                });
            }

            if(uploaded.result === 'error') {
                spinner.close();
                this.$buefy.dialog.alert({
                    title: 'Error',
                    message: uploaded.message,
                    type: 'is-danger',
                    hasIcon: true,
                    icon: 'times-circle',
                    iconPack: 'fa',
                    ariaRole: 'alertdialog',
                    ariaModal: true
                });
            }
            else {
                for(url of uploaded) {
                    this.$emit("url", url);
                }

                spinner.close();
            }
        }
    }
}
</script>

<style lang="scss" scoped>

</style>
