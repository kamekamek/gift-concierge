import { useCallback, useEffect, useRef } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';

const WS_URL = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3001/ws';

export function useChatWebSocket() {
  const { sendMessage: wssSendMessage, lastMessage, readyState } = useWebSocket(WS_URL, {
    shouldReconnect: (closeEvent) => true,
    reconnectAttempts: 10,
    reconnectInterval: 3000,
    onOpen: () => {
      console.log('WebSocket接続が確立されました');
    },
    onClose: () => {
      console.log('WebSocket接続が切断されました');
    },
    onError: (error) => {
      console.error('WebSocket接続エラー:', error);
    },
  });

  const sendMessage = useCallback((message: string) => {
    console.log('送信メッセージ:', message);
    wssSendMessage(message);
  }, [wssSendMessage]);

  const connectionStatus = {
    [ReadyState.CONNECTING]: '接続中...',
    [ReadyState.OPEN]: '接続済み',
    [ReadyState.CLOSING]: '切断中...',
    [ReadyState.CLOSED]: '切断',
    [ReadyState.UNINSTANTIATED]: '未接続',
  }[readyState];

  useEffect(() => {
    console.log('WebSocket status:', connectionStatus);
  }, [connectionStatus]);

  useEffect(() => {
    if (lastMessage) {
      console.log('受信メッセージ:', lastMessage);
    }
  }, [lastMessage]);

  return {
    sendMessage,
    lastMessage,
    readyState,
    connectionStatus,
  };
} 