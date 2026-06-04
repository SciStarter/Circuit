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
                uploaded = await this.$axios.$post("/api/upload", form, this.$store.state.auth);
            }
            catch(err) {
                console.error(err.toJSON());
                spinner.close();
                this.$buefy.dialog.alert({
                    title: 'Error',
                    message: "Unable to communicate with the server: " + (!!err.response ? err.response.data.message : "no response"),
                    type: 'is-danger',
                    hasIcon: false,
                    ariaRole: 'alertdialog',
                    ariaModal: true
                });
                return;
            }

            if(uploaded.result === 'error') {
                console.error(uploaded);
                spinner.close();
                this.$buefy.dialog.alert({
                    title: 'Error',
                    message: uploaded.message,
                    type: 'is-danger',
                    hasIcon: false,
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
