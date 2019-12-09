import React from 'react'
import style from './index.module.css'
import messageStyle from '../Message/index.module.css'
import { Message } from '../Message'
import Linkify from 'react-linkify'

const emptyList = (
  <div className={style.empty}>
    <span role='img' aria-label='post'>
      📝
    </span>
    <h2>No Messages Yet</h2>
    <p>Be the first to post in this conversation!</p>
  </div>
)

export const MessageList = ({ messages = [], user, users, getMessages, conversation}) => (
  <ul id='messages' className={style.component}>
    {
      messages.length > 0 ? (
        <wrapper->
          <li key="click-for-more" className={messageStyle.component}>
            <div>
              <span onClick={() => getMessages({conversationId: conversation.id, limit: 3, since: messages.sort((a, b) => { return b.createdAt - a.createdAt })[0].id})}>Click to load 3 more</span>
            </div>
          </li>
          {
            messages
              .sort((a, b) => { return b.createdAt - a.createdAt })
              .map(message => Message({ user, users })(message))
          }
        </wrapper->
      ) : (
        emptyList
      )
    }
  </ul>
)
