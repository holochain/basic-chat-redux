(window.webpackJsonp=window.webpackJsonp||[]).push([[0],{187:function(e,t,n){"use strict";n.r(t);var a=n(17),r=n(24),o=n(75),s=n(76),c=n(91),l=n(77),i=n(92),m=n(0),u=n.n(m),p=n(78),d=n.n(p),g=(n(98),n(79)),h=n(80),f=n.n(h),E=function(e){var t=e.user,n=void 0===t?{}:t;return u.a.createElement("header",{className:f.a.component},u.a.createElement("img",{src:n.avatarURL?n.avatarURL:"data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7",alt:n.name?n.name[0]:""}),u.a.createElement("div",null,u.a.createElement("h3",null,n.name),u.a.createElement("h5",null,n.id&&"@".concat(n.id.substring(0,15)))))},v=n(81),_=n.n(v),C=function(e){var t=e.room,n=(e.current,e.users);return u.a.createElement("ul",{className:_.a.component},t.users.map(function(e){return u.a.createElement("li",{key:e},u.a.createElement("img",{src:n[e]?n[e].avatar_url:"",alt:n[e]?n[e].name[0]:""}),u.a.createElement("p",null,n[e]?n[e].name:"?"),u.a.createElement("h5",null,"\xa0| ",e&&"@".concat(e.substring(0,15))))}))},b=n(41),L=n.n(b),k=n(82),A=n.n(k),O=n(83),R=n.n(O),x=function(e){e.user;var t=e.users;return function(e){var n=t[e.sender]||{name:"?",avatar_url:""};return e.sender?u.a.createElement("li",{key:e.id,className:A.a.component},u.a.createElement("img",{src:n.avatar_url,alt:n.name[0]}),u.a.createElement("div",null,u.a.createElement("span",null,"".concat(n.name," | ").concat(function(e){var t=new Date(1e3*e),n=t.getMinutes();return"".concat(t.getHours(),":").concat(n<10?"0"+n:n)}(e.createdAt))),u.a.createElement("p",null,u.a.createElement(R.a,{properties:{target:"_blank"}},e.text)))):null}},M=u.a.createElement("div",{className:L.a.empty},u.a.createElement("span",{role:"img","aria-label":"post"},"\ud83d\udcdd"),u.a.createElement("h2",null,"No Messages Yet"),u.a.createElement("p",null,"Be the first to post in this room!")),y=function(e){var t=e.messages,n=void 0===t?[]:t,a=e.user,r=e.users,o=e.createConvo;return u.a.createElement("ul",{id:"messages",className:L.a.component},n.length>0?u.a.createElement("wrapper-",null,n.sort(function(e,t){return t.createdAt-e.createdAt}).map(function(e){return x({user:a,createConvo:o,users:r})(e)})):M)},U=n(84),S=n.n(U),H=function(e){var t=e.state,n=(t.user,t.room),a=void 0===n?{}:n,r=(t.message,e.actions),o=r.sendMessage,s=r.runCommand,c=r.getMessages;return a.id?u.a.createElement("form",{className:S.a.component,onSubmit:function(e){e.preventDefault();var t=e.target[0].value.trim();0!==t.length&&(e.target[0].value="",t.startsWith("/")?s(t.slice(1)):o({text:t,roomId:a.id}))}},u.a.createElement("input",{placeholder:"Type a Message.."}),u.a.createElement("button",{type:"submit"},u.a.createElement("svg",null,u.a.createElement("use",{xlinkHref:"index.svg#send"}))),u.a.createElement("button",{type:"submit",onClick:function(){return c(a.id)}},u.a.createElement("img",{src:"refresh.svg",alt:""}))):null},w=n(85),j=n.n(w),N=function(e){var t,n=e.rooms,a=void 0===n?[]:n,r=e.user,o=e.messages,s=e.current,c=e.actions;return t=r.id?u.a.createElement("li",{onClick:c.getRooms},u.a.createElement("input",{type:"image",alt:"refresh",src:"refresh.svg"})):u.a.createElement("li",null),u.a.createElement("ul",{className:j.a.component},a.map(function(e){var t,n=Object.keys(o[e.id]||{}),a=n.length>0&&o[e.id][n.pop()],l=e.users.find(function(e){return e.id!==r.id});return u.a.createElement("li",{key:e.id,disabled:e.id===s.id,onClick:function(t){return c.joinRoom(e)}},e.name.match(r.id)&&l?u.a.createElement("img",{src:l.avatarURL,alt:l.id}):(t=e.isPrivate?"lock":"public",u.a.createElement("svg",null,u.a.createElement("use",{xlinkHref:"index.svg#".concat(t)}))),u.a.createElement("col-",null,u.a.createElement("p",null,e.name.replace(r.id,"")),u.a.createElement("span",null,a&&a.text)))}),t)},T=n(86),B=n.n(T),D=function(e){var t=e.state,n=t.room,a=t.user,r=t.sidebarOpen,o=t.userListOpen,s=e.actions,c=s.setSidebar,l=s.setUserList;return u.a.createElement("header",{className:B.a.component},u.a.createElement("button",{onClick:function(e){return c(!r)}},u.a.createElement("svg",null,u.a.createElement("use",{xlinkHref:"index.svg#menu"}))),u.a.createElement("h1",null,n.name&&n.name.replace(a.id,"")),n.users&&u.a.createElement("div",{onClick:function(e){return l(!o)}},u.a.createElement("span",null,n.users.length),u.a.createElement("svg",null,u.a.createElement("use",{xlinkHref:"index.svg#members"}))))},I=n(87),J=n.n(I),W=function(e){var t=e.submit;return u.a.createElement("form",{className:J.a.component,onSubmit:function(e){e.preventDefault();var n=e.target[0].value;0!==n.length&&(t({name:n}),e.target[0].value="")}},u.a.createElement("input",{placeholder:"Create a Room"}),u.a.createElement("button",{type:"submit"},u.a.createElement("svg",null,u.a.createElement("use",{xlinkHref:"index.svg#add"}))))},P=n(88),F=n.n(P),K=function(e){var t=e.message;return u.a.createElement("section",null,u.a.createElement("div",{className:F.a.component},u.a.createElement("span",{role:"img","aria-label":"post"},u.a.createElement("svg",{viewBox:"0 0 59 80"},u.a.createElement("g",null,u.a.createElement("path",{d:"M48.417 49.444L38.585 73.229 36.012 71.394 41.802 56.726 31.211 59.519C31.211 59.519 26.022 42.42 26.513 30.837 26.218 22.744 27.644 16.96 30.167 13.6 34.274 8.408 42.099 3.953 49.793 7.645 54.701 9.869 57.658 13.889 58.107 17.824 59.479 34.859 38.498 41.817 38.498 41.817L38.085 38.191C38.085 38.191 44.945 35.464 49.105 29.553 53.266 23.643 51.245 15.88 46.569 14.464 42.061 13.577 37.34 18.757 36.516 27.966 34.721 41.776 39.393 52.006 39.393 52.006L48.417 49.444M0 62.396L15.402 74.372 16.522 71.783 8.26 65.71 16.853 63.305C16.853 63.305 10.998 47.535 10.597 32.251 10.229 27.354 9.896 11.684 15.924 8.975 16.976 7.213 18.743 5.903 18.743 5.903 18.743 5.903 3.467 4.584 4.184 32.453 5.706 51.166 9.451 59.821 9.451 59.821L0 62.396"}),u.a.createElement("path",{d:"M29.91,61.761 L38.969,59.263 L30.045,80 L11.998,66.538 L19.098,64.644 C19.098,64.644 3.276,20.694 21.128,5.095 C32.325,-3.789 43.164,3.999 42.965,4.872 C40.203,4.812 37.842,5.844 37.842,5.844 C37.842,5.844 31.789,3.265 27.475,12.503 C23.719,20.705 22.048,34.474 29.91,61.761"}),u.a.createElement("path",{d:"M41.806,17.606 C42.599,24.356 38.244,27.206 38.244,27.206 L37.819,30.659 C37.819,30.659 47.709,25.549 45.669,15.819 C44.566,15.913 42.212,17.242 41.806,17.606"})))),u.a.createElement("p",null,t)))},Q=n(89),q=n.n(Q),G=function(){return u.a.createElement("section",null,u.a.createElement("div",{className:q.a.component},u.a.createElement("span",{role:"img","aria-label":"post"},u.a.createElement("svg",{viewBox:"0 0 59 80"},u.a.createElement("g",null,u.a.createElement("path",{d:"M48.417 49.444L38.585 73.229 36.012 71.394 41.802 56.726 31.211 59.519C31.211 59.519 26.022 42.42 26.513 30.837 26.218 22.744 27.644 16.96 30.167 13.6 34.274 8.408 42.099 3.953 49.793 7.645 54.701 9.869 57.658 13.889 58.107 17.824 59.479 34.859 38.498 41.817 38.498 41.817L38.085 38.191C38.085 38.191 44.945 35.464 49.105 29.553 53.266 23.643 51.245 15.88 46.569 14.464 42.061 13.577 37.34 18.757 36.516 27.966 34.721 41.776 39.393 52.006 39.393 52.006L48.417 49.444M0 62.396L15.402 74.372 16.522 71.783 8.26 65.71 16.853 63.305C16.853 63.305 10.998 47.535 10.597 32.251 10.229 27.354 9.896 11.684 15.924 8.975 16.976 7.213 18.743 5.903 18.743 5.903 18.743 5.903 3.467 4.584 4.184 32.453 5.706 51.166 9.451 59.821 9.451 59.821L0 62.396"}),u.a.createElement("path",{d:"M29.91,61.761 L38.969,59.263 L30.045,80 L11.998,66.538 L19.098,64.644 C19.098,64.644 3.276,20.694 21.128,5.095 C32.325,-3.789 43.164,3.999 42.965,4.872 C40.203,4.812 37.842,5.844 37.842,5.844 C37.842,5.844 31.789,3.265 27.475,12.503 C23.719,20.705 22.048,34.474 29.91,61.761"}),u.a.createElement("path",{d:"M41.806,17.606 C42.599,24.356 38.244,27.206 38.244,27.206 L37.819,30.659 C37.819,30.659 47.709,25.549 45.669,15.819 C44.566,15.913 42.212,17.242 41.806,17.606"})))),u.a.createElement("p",null,"Join a room from the left or create a new room using the field on the bottom left")))},V=n(90),Y=n.n(V),z=function(e){var t=e.registerUser;return u.a.createElement("section",null,u.a.createElement("div",{className:Y.a.component},u.a.createElement("span",{role:"img","aria-label":"post"},u.a.createElement("svg",{viewBox:"0 0 59 80"},u.a.createElement("g",null,u.a.createElement("path",{d:"M48.417 49.444L38.585 73.229 36.012 71.394 41.802 56.726 31.211 59.519C31.211 59.519 26.022 42.42 26.513 30.837 26.218 22.744 27.644 16.96 30.167 13.6 34.274 8.408 42.099 3.953 49.793 7.645 54.701 9.869 57.658 13.889 58.107 17.824 59.479 34.859 38.498 41.817 38.498 41.817L38.085 38.191C38.085 38.191 44.945 35.464 49.105 29.553 53.266 23.643 51.245 15.88 46.569 14.464 42.061 13.577 37.34 18.757 36.516 27.966 34.721 41.776 39.393 52.006 39.393 52.006L48.417 49.444M0 62.396L15.402 74.372 16.522 71.783 8.26 65.71 16.853 63.305C16.853 63.305 10.998 47.535 10.597 32.251 10.229 27.354 9.896 11.684 15.924 8.975 16.976 7.213 18.743 5.903 18.743 5.903 18.743 5.903 3.467 4.584 4.184 32.453 5.706 51.166 9.451 59.821 9.451 59.821L0 62.396"}),u.a.createElement("path",{d:"M29.91,61.761 L38.969,59.263 L30.045,80 L11.998,66.538 L19.098,64.644 C19.098,64.644 3.276,20.694 21.128,5.095 C32.325,-3.789 43.164,3.999 42.965,4.872 C40.203,4.812 37.842,5.844 37.842,5.844 C37.842,5.844 31.789,3.265 27.475,12.503 C23.719,20.705 22.048,34.474 29.91,61.761"}),u.a.createElement("path",{d:"M41.806,17.606 C42.599,24.356 38.244,27.206 38.244,27.206 L37.819,30.659 C37.819,30.659 47.709,25.549 45.669,15.819 C44.566,15.913 42.212,17.242 41.806,17.606"})))),u.a.createElement("p",null,"It looks like this is the first time using chat with this agent. Register a handle and avatar for this agent ID."),u.a.createElement("form",{onSubmit:function(e){e.preventDefault();var n=e.target[0].value,a=e.target[1].value;0!==n.length&&t({name:n,avatarURL:a})}},u.a.createElement("input",{placeholder:"input @handle"}),u.a.createElement("br",null),u.a.createElement("input",{placeholder:"paste url for avatar image"}),u.a.createElement("br",null),u.a.createElement("button",{type:"submit"},"Register!"))))},X=function(e){function t(e){var n;return Object(o.a)(this,t),(n=Object(c.a)(this,Object(l.a)(t).call(this,e))).state={holochainConnection:Object(g.connect)("ws://localhost:3402"),connected:!1,user:{},users:{},room:{},rooms:[],messages:{},sidebarOpen:!1,userListOpen:window.innerWidth>1e3},n.actions={setSidebar:function(e){return n.setState({sidebarOpen:e})},setUserList:function(e){return n.setState({userListOpen:e})},setUser:function(e){n.setState({user:e}),n.actions.getRooms()},setRoom:function(e){n.setState({room:e,sidebarOpen:!1}),n.actions.getMessages(e.id),n.actions.getRoomMembers(e.id),n.actions.scrollToEnd()},joinRoom:function(e){console.log("joining room"),n.actions.setRoom(e),n.makeHolochainCall("holo-chat/chat/join_stream",{stream_address:e.id},function(e){console.log("joined room",e)})},getRoomMembers:function(e){n.makeHolochainCall("holo-chat/chat/get_members",{stream_address:e},function(e){console.log("retrieved members",e);var t=e.Ok;t.forEach(function(e){n.actions.getUserProfile(e)}),n.setState({room:Object(r.a)({},n.state.room,{users:t})})})},sendMessage:function(e){var t=e.text,a=e.roomId,r={message_type:"text",timestamp:Math.floor(Date.now()/1e3),payload:t,meta:""};n.makeHolochainCall("holo-chat/chat/post_message",{stream_address:a,message:r},function(e){console.log("message posted",e),setTimeout(function(){return n.actions.getMessages(a)},1e3),n.actions.scrollToEnd()})},getMessages:function(e){n.makeHolochainCall("holo-chat/chat/get_messages",{address:e},function(t){console.log("retrieved messages",t);var o=t.Ok.map(function(e){var t=e.address,n=e.entry;return{text:n.payload,sender:n.author,createdAt:n.timestamp,id:t}});n.setState({messages:Object(r.a)({},n.state.messages,Object(a.a)({},e,o))})})},createRoom:function(e){console.log(e);var t={name:e.name,description:"",initial_members:[]};n.makeHolochainCall("holo-chat/chat/create_stream",t,function(t){console.log("created room",t),n.actions.setRoom({id:t.Ok,name:e.name,users:[]}),n.actions.getRooms()})},getUserProfile:function(e){n.makeHolochainCall("holo-chat/chat/get_member_profile",{agent_address:e},function(t){console.log("retrieved profile",t),n.setState({users:Object(r.a)({},n.state.users,Object(a.a)({},e,t.Ok))})})},getRooms:function(){n.makeHolochainCall("holo-chat/chat/get_all_public_streams",{},function(e){console.log("retrieved public rooms",e);var t=e.Ok.map(function(e){var t=e.address,n=e.entry;return{id:t,private:!n.public,name:n.name,users:[]}});n.setState({rooms:t})})},registerUser:function(e){var t=e.name,a=e.avatarURL;n.makeHolochainCall("holo-chat/chat/register",{name:t,avatar_url:a},function(e){console.log("registered user",e),n.actions.setUser({id:e.Ok,name:t,avatarURL:a})})},scrollToEnd:function(e){return setTimeout(function(){var e=document.querySelector("#messages");e&&(e.scrollTop=1e5)},0)}},n}return Object(i.a)(t,e),Object(s.a)(t,[{key:"componentDidMount",value:function(){var e=this;this.state.holochainConnection.then(function(t){(0,t.call)("holo-chat/chat/get_my_member_profile")({}).then(function(t){var n=JSON.parse(t).Ok;n?(console.log("registration user found with profile:",n),e.actions.setUser({id:n.address,name:n.name,avatarURL:n.avatar_url})):console.log("User has not registered a profile. Complete the form to proceed"),e.setState({connected:!0})})})}},{key:"makeHolochainCall",value:function(e,t,n){this.state.holochainConnection.then(function(a){(0,a.call)(e)(t).then(function(e){return n(JSON.parse(e))})})}},{key:"render",value:function(){var e=this.state,t=e.user,n=e.users,a=e.room,r=e.rooms,o=e.messages,s=e.sidebarOpen,c=e.userListOpen,l=e.connected,i=this.actions,m=i.createRoom,p=i.registerUser;return u.a.createElement("main",null,u.a.createElement("aside",{"data-open":s},u.a.createElement(E,{user:t}),u.a.createElement(N,{user:t,rooms:r,messages:o,current:a,actions:this.actions}),t.id&&u.a.createElement(W,{submit:m})),u.a.createElement("section",null,u.a.createElement(D,{state:this.state,actions:this.actions}),a.id?u.a.createElement("row-",null,u.a.createElement("col-",null,u.a.createElement(y,{user:t,users:n,messages:o[a.id]}),u.a.createElement(H,{state:this.state,actions:this.actions})),c&&u.a.createElement(C,{room:a,current:t.id,users:n})):l?t.id?u.a.createElement(G,null):u.a.createElement(z,{registerUser:p}):u.a.createElement(K,{message:"Connecting to Holochain... Make sure the conductor is running and try refreshing the page"})))}}]),t}(u.a.Component);d.a.render(u.a.createElement(X,null),document.querySelector("#root"))},41:function(e,t,n){e.exports={component:"MessageList_component__HVOTE",empty:"MessageList_empty__dHcOm"}},80:function(e,t,n){e.exports={component:"UserHeader_component__kYFop",pulse:"UserHeader_pulse__1n84i"}},81:function(e,t,n){e.exports={component:"UserList_component__2KpNP",hint:"UserList_hint__1K7w2",online:"UserList_online__3Ivmn"}},82:function(e,t,n){e.exports={component:"Message_component__TC029",online:"Message_online__37W_w"}},84:function(e,t,n){e.exports={component:"CreateMessageForm_component__AtCV9"}},85:function(e,t,n){e.exports={component:"RoomList_component__3s6KT"}},86:function(e,t,n){e.exports={component:"RoomHeader_component__1fLym"}},87:function(e,t,n){e.exports={component:"CreateRoomForm_component__3kony"}},88:function(e,t,n){e.exports={component:"WelcomeScreen_component__2asIb",pulse:"WelcomeScreen_pulse__G64pa"}},89:function(e,t,n){e.exports={component:"JoinRoomScreen_component__UNHCf",pulse:"JoinRoomScreen_pulse__2iQbC"}},90:function(e,t,n){e.exports={component:"RegisterScreen_component__2cwRB",pulse:"RegisterScreen_pulse__QWETo"}},93:function(e,t,n){e.exports=n(187)},98:function(e,t,n){}},[[93,1,2]]]);
//# sourceMappingURL=main.3b202581.chunk.js.map