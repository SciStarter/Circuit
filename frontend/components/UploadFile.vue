<template>
<div class="upload-file">
  <b-upload @input="upload" native drag-drop>
    <section class="section">
      <div class="content has-text-centered">
        <p>
          <b-icon
            icon="upload"
            size="is-large">
          </b-icon>
        </p>
        <p>Drop your files here or click to upload</p>
      </div>
    </section>
  </b-upload>
</div>
</template>

<script>
export default {
    name: "UploadFile",

    methods: {
        async upload(file) {
            let spinner = this.$buefy.loading.open();
            let uploaded = null;

            let form = new FormData();
            form.append("file", file);

            try {
                uploaded = await this.$axios.$post("/api/upload", form);
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
                for(let url of uploaded) {
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
