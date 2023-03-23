<template>
    <div class="item-wrapper">
        <img src="~/assets/delete.png" alt="" id="delete" @click="deleteItem()">
        <img src="~/assets/share.png" alt="" id="share" @click="shareItem()">
        <div class="image-container">
            <img src="~/assets/placeholder.png" alt="">
        </div>
        <div class="info-container">
            <span>{{ filename }}</span>
            <span id="size">{{ (size / 1048576).toFixed(2) }} MB</span>
        </div>
    </div>
</template>

<style scoped src="~/styles/item.css"></style>

<script>
export default {
    props: {
        id: {
            type:String,
            required:true
        },
        filename: {
            type:String,
            required:true
        },
        size: {
            type:Number,
            required:true
        },
        owned_by: {
            type:String
        },
        path: {
            type:String
        }
    },
    methods:{
        async deleteItem() {
            const data = {
                _id:this.id
            };
            const result = await $fetch('http://localhost:4200/delete', {
                mode:"cors",
                method: "POST",
                credentials:"include",
                body: data
            });
            // refresh the page on successful deletion
            if(result.deletedCount > 0) {
                window.location.reload();
            }
        },
        shareItem() {
            let link = "http://localhost:4200/share/" + this.id;
            navigator.clipboard.writeText(link.toString()).then(() => alert("copied to clipboard!"));
        }
    }
}
</script>


