import { useCallback, useEffect, useRef } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';

const WS_URL = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3000/ws';

export function useWebSocket() {
  const { sendMessage, lastMessage, readyState } = useWebSocket(WS_URL, {
    shouldReconnect: (closeEvent) => true,
    reconnectAttempts: 10,
    reconnectInterval: 3000,
  });

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

  return {
    sendMessage,
    lastMessage,
    readyState,
    connectionStatus,
  };
} 