<template>
    <div class="container centered">
        <div class="centered fancy-bg">
            <h1>hey there!</h1>
        <form action="" method="POST" v-on:submit.prevent="submit()" class="centered">
            <input type="text" placeholder="name" v-model="name">
            <input type="text" placeholder="email" v-model="email">
            <input type="password" placeholder="password" v-model="password">
            <button type="submit">register</button>
        </form>
        <a href="/login">already got an account?</a>
        </div>
        <div v-if="success == true" class="success">
            <a href="/login">success! return to <span class="turqoise">login</span></a>
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
            success:undefined
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
            const result = await $fetch('http://localhost:4200/user/new', {
                mode:"cors",
                method:"POST",
                credentials:"include",
                body:req
            });
            if(result.insertedId) {
                this.success = true;
            }
        }
    }
}
</script>