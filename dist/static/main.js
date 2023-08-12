console.log('你好世界');
let access_token = 'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJ1aWQiOiJhMSIsInVzZXJuYW1lIjoibXktYWRtaW4iLCJleHAiOjE2OTE1NzMzNDN9.kPi2aaNcg00rXxe04XNT7izKKxTH41BwG8QRYCXsQE5mZdauvGJ8L4MXu0jaYIaB0Cqe5HYVjm74I67j_h6Pnw';
let refresh_token = '';

(function () {
  const req = new XMLHttpRequest();
  req.open('get', '/api/auth?username=my-admin&password=1324123', false)
  req.onload = (ev => {
    access_token = JSON.parse(ev.target.response)['access_token'] || ''
    refresh_token = JSON.parse(ev.target.response)['refresh_token'] || ''
  })
  req.send()
  console.log('请求已发送')
})();

(function () {
  const req = new XMLHttpRequest();
  req.open('get', '/api/user')
  req.responseType = 'json';
  const token = access_token || '';
  req.setRequestHeader('Authorization', 'Bearer ' + token);
  req.onload = (ev => {
    console.log(ev.target.response)
  })
  req.send()
  console.log('请求已发送')
})();

function refreshToken() {
  const req = new XMLHttpRequest();
  req.open('get', '/api/refresh?refresh_token=' + refresh_token)
  req.onload = (ev => {
    refresh_token = JSON.parse(ev.target.response)['refresh_token'] || ''
    access_token = JSON.parse(ev.target.response)['access_token'] || ''
  })
  req.send()
  console.log('请求已发送')
}
