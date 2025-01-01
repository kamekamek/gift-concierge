import { useCallback, useEffect, useRef } from 'react';
import useWebSocket from 'react-use-websocket';

const WS_URL = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3001/ws';

export function useChatWebSocket() {
  const { sendMessage, lastMessage, readyState } = useWebSocket(WS_URL, {
    shouldReconnect: (closeEvent) => true,
    reconnectAttempts: 10,
    reconnectInterval: 3000,
  });

  const connectionStatus = {
    [useWebSocket.ReadyState.CONNECTING]: '接続中...',
    [useWebSocket.ReadyState.OPEN]: '接続済み',
    [useWebSocket.ReadyState.CLOSING]: '切断中...',
    [useWebSocket.ReadyState.CLOSED]: '切断',
    [useWebSocket.ReadyState.UNINSTANTIATED]: '未接続',
  }[readyState];

  useEffect(() => {
    console.log('WebSocket status:', connectionStatus);
  }, [connectionStatus]);

  return {
    sendMessage,
    lastMessage,
    readyState,
    connectionStatus,
  };
} 