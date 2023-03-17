<template>
    <div class="header">
        <div class="header-bg"></div>
        <span id="logo">
            | nuclear |
        </span>
        <div class="nav">
            <a href="/">[home]</a>
            <a href="/dashboard">[dashboard]</a>
        </div>
        <div class="user">
            <a href="/register">[register]</a>
        </div>
    </div>
</template>

<style src="~/styles/header.css">
</style>

<script>
export default {
    async verify() {
        const result = await $fetch('http://localhost:4200/auth',{credentials:"include"});
        if(result == false) {
            navigateTo('/login');
        }
    }
}
</script>