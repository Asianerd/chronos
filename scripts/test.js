// fetch("http://127.0.0.1:8000/post_test", {
//     method:"POST",
//     body: "rahhh",
//     headers: {
//         "Content-type": "application/json; charset=UTF-8"
//     }
// });

sendPostRequest(`${BACKEND_ADDRESS}/post_test`, JSON.stringify({
    "username":"lorem_ipsum",
    "password":"ps"
}), (response) => {
    console.log(response);
})
