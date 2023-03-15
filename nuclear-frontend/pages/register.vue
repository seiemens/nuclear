<template>
    <div class="container centered">
        <h1>hey there!</h1>
        <form action="" method="POST" v-on:submit.prevent="submit()" class="centered">
            <input type="text" placeholder="name" v-model="name">
            <input type="text" placeholder="email" v-model="email">
            <input type="password" placeholder="password" v-model="password">
            <button type="submit">login</button>
        </form>
        <a href="/login">already got an account?</a>
        <div v-if="res == true" class="green">
            <a href="/login">success, go back to login</a>
        </div>
    </div>
</template>

<script>
export default {
    data(){
        return {
            name:"",
            email:"",
            password:"",
            res:undefined
        }
    },
    methods:{
        async submit(){
            let req = {
                name:this.name,
                email:this.email,
                role:0,
                password:this.password
            };
            const {result} = await $fetch('http://localhost:4200/user/new', {
                mode:"no-cors",
                method:"POST",
                body:req
            });
            console.log(result);
        }
    }
}
</script>