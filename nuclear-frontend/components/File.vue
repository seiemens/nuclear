<template>
    <div class="item-wrapper">
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
        }
    }
}
</script>