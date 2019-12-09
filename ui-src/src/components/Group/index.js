import React from 'react'
import '../../index.css'
import { UserHeader } from '../UserHeader'
import { UserList } from '../UserList'
import { MessageList } from '../MessageList'
import { CreateMessageForm } from '../CreateMessageForm'
import { ConversationList } from '../ConversationList'
import { ConversationHeader } from '../ConversationHeader'
import { StartConversationForm } from '../StartConversationForm'
import { WelcomeScreen } from '../WelcomeScreen'
import { JoinConversationScreen } from '../JoinConversationScreen'
import { RegisterScreen } from '../RegisterScreen'

export const Group = ({
  groups = [],
  currentGroup = {},
  joinGroup,
  sidebarOpen,
  user = {},
  users = [],
  conversations = [],
  messages = [],
  conversation = {},
  getConversations,
  startConversation,
  joinConversation,
  setSidebar,
  sendMessage,
  runCommand,
  getMessages,
  userListOpen,
  setFullName,
  connected,
  registerUser,
  setUserList
}) => (
  <main>
    <aside data-open={sidebarOpen}>
      <UserHeader user={user} />
        <ConversationList
          user={user}
          conversations={conversations}
          messages={messages}
          current={conversation}
          getConversations={getConversations}
          joinConversation={joinConversation}
        />
      {user.id && <StartConversationForm submit={startConversation} currentGroup={currentGroup}/>}
    </aside>
    <section>
      <ConversationHeader
        conversation={conversation}
        user={user}
        sidebarOpen={sidebarOpen}
        userListOpen={userListOpen}
        setSidebar={setSidebar}
        setUserList={setUserList}
       />
       {conversation.id ? (
          <row->
            <col->
              <MessageList
                user={user}
                users={users}
                messages={messages[conversation.id]}
                getMessages={getMessages}
                conversation={conversation}
              />
              <CreateMessageForm user={user} conversation={conversation} message={''} runCommand={runCommand} sendMessage={sendMessage} getMessages={getMessages}/>
            </col->
            {userListOpen && (
              <UserList conversation={conversation} users={users} />
            )}
          </row->
        ) : connected ? (
          user.id ? <JoinConversationScreen /> : <RegisterScreen registerUser={registerUser} />
        ) : (
          <WelcomeScreen message="Please check your Holochain conductor is running and accepting websocket connections on the correct port" />
        )}
    </section>
  </main>
)
