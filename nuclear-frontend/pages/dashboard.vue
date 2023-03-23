<template>
    <div class="container floating">
        <h1>my files</h1>
        <div class="centered upload-container">
            <form v-on:submit.prevent="newFile()">
                <label id="upload">select
                <input 
                    type="file" 
                    v-bind="upload"
                    id="fileUpload"
                >
            </label>
            <button type="submit" class="upload">send it</button>
            </form>
        </div>
        <div class="item-container">
            <File 
            v-for="f in files" 
            :key="f._id"
            :id="f._id.$oid"
            :filename="f.name"
            :size="f.size"
            :owned_by="f.owned_by"
            :path="f.path"
            />
        </div>
    </div>
</template>

<style src="~/styles/item.css"></style>

<script>
export default {
    data() {
        return {
            files: [],
            upload:undefined
        };
    },
    async mounted() {
        auth();
        const result = await $fetch('http://localhost:4200/files', {credentials:"include"});
        this.files = result;
    },
    methods:{
        async newFile(){
            let f = document.getElementById("fileUpload").files[0];
            let formdata = new FormData();
            formdata.set('file', f);
            const res = await $fetch('http://localhost:4200/upload', {method:"POST", credentials:"include", body:formdata});
            // refresh the page on successful deletion
            if(res.insertedId) {
                window.location.reload();
            }
       }
    }
}
</script>