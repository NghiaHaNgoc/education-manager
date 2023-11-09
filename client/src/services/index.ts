export const accessToken = () => {
    console.log(localStorage.getItem('user'));
    return JSON.parse(localStorage.getItem('user') as string);
}

export const config = () => ({
    headers : {
        "Content-Type" : "application/json",
        "Authorization" : `Bearer ${accessToken()?.token}`
    }
})