<template>
    <div class="container floating">
        <h1>my files</h1>
        <div class="centered">
            <input type="file" multiple>
        </div>
        <div class="container">
            <File 
            v-for="f in files" 
            :key="f._id"
            :id="f._id"
            :name="f.name"
            :size="f.size"
            />
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            files: []
        };
    },
    mounted() {
        auth();
    },
    async fetch() {
        const result = await $fetch('http://localhost:4200/files', {credentials:"include"});
        this.files = result.files;
    }
}
</script>