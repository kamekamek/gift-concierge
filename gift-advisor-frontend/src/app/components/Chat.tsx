import { useEffect, useRef, useState } from 'react';
import { useWebSocket } from '../hooks/useWebSocket';

interface Message {
  user_id: string;
  message: string;
  message_type: 'user_message' | 'bot_response' | 'typing' | 'error';
}

export default function Chat() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { sendMessage, lastMessage, readyState } = useWebSocket();

  useEffect(() => {
    if (lastMessage) {
      const message = JSON.parse(lastMessage.data) as Message;
      setMessages(prev => [...prev, message]);
    }
  }, [lastMessage]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim()) return;

    const message: Message = {
      user_id: 'user',
      message: input,
      message_type: 'user_message',
    };

    sendMessage(JSON.stringify(message));
    setMessages(prev => [...prev, message]);
    setInput('');
  };

  return (
    <div className="flex flex-col h-screen bg-gray-100">
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.map((msg, index) => (
          <div
            key={index}
            className={`flex ${
              msg.message_type === 'user_message' ? 'justify-end' : 'justify-start'
            }`}
          >
            <div
              className={`max-w-[70%] rounded-lg p-3 ${
                msg.message_type === 'user_message'
                  ? 'bg-blue-500 text-white'
                  : msg.message_type === 'error'
                  ? 'bg-red-500 text-white'
                  : 'bg-white'
              }`}
            >
              {msg.message_type === 'typing' ? (
                <div className="flex space-x-2">
                  <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce" />
                  <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce delay-100" />
                  <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce delay-200" />
                </div>
              ) : (
                msg.message
              )}
            </div>
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>

      <form onSubmit={handleSubmit} className="p-4 bg-white border-t">
        <div className="flex space-x-4">
          <input
            type="text"
            value={input}
            onChange={e => setInput(e.target.value)}
            placeholder="メッセージを入力..."
            className="flex-1 p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            type="submit"
            className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            送信
          </button>
        </div>
      </form>
    </div>
  );
} 