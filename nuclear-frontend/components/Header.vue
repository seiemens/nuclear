<template>
    <div class="header">
            <span id="logo">
                | nuclear |
            </span>
            <a href="/">[home]</a>
            <a href="/dashboard">[dashboard]</a>
            <div class="profile">
                <a v-if="!auth" href="/register">[register]</a>
                <a v-else href="/profile">[profile]</a>
            </div>
    </div>
</template>

<style src="~/styles/header.css">
</style>

<script>
export default {
    data() {
        return{
            auth: false
        }
    },
    mounted() {
        const cookie = useCookie('auth_biscuit');
        console.log(cookie);
        if(cookie.value != undefined) {
            this.auth = true;
        }
    }
}
</script>