<template>
    <div class="item-wrapper">

    </div>
</template>

<style src="~/styles/item.css"></style>

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