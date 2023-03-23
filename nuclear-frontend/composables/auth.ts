/// authentication composable
export default async function() {
    const result = await $fetch('http://localhost:4200/auth',{credentials:"include"});
    if(result == false) {
        navigateTo('/login');
    } 
}