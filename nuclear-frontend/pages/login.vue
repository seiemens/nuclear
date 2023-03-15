<template>
    <div class="container centered">
        <h1>welcome back!</h1>
        <form v-on:submit.prevent="onSubmit()" class="centered">
            <input type="text" placeholder="email" v-model="email">
            <input type="password" placeholder="password" v-model="password">
            <button type="submit">login</button>
        </form>
        <a href="/register">no account yet?</a>
    </div>
</template>

<script>
export default {
    data(){
        return {
            email:"",
            password:""
        }
    },
    methods:{
        async onSubmit(){
            let reqData = {
                email:this.email,
                password:this.password
            };
            const result = await $fetch('http://127.0.0.1:4200/login', {
                mode:"cors",
                method: "POST",
                credentials:"include",
                body:reqData
            });
            console.log(result);
        }
    }
}
</script>