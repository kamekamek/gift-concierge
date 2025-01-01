import { useCallback, useEffect, useRef } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import { useIsClient } from './useIsClient';

const WS_URL = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3001/ws';

export function useChatWebSocket() {
  const isClient = useIsClient();
  
  const { sendMessage: wssSendMessage, lastMessage, readyState } = useWebSocket(
    isClient ? WS_URL : null,
    {
      shouldReconnect: (closeEvent) => true,
      reconnectAttempts: 10,
      reconnectInterval: 3000,
      onOpen: () => {
        console.log('WebSocket接続が確立されました');
      },
      onClose: (event) => {
        console.log('WebSocket接続が切断されました', {
          code: event.code,
          reason: event.reason,
          wasClean: event.wasClean
        });
      },
      onError: (event: Event) => {
        console.error('WebSocket接続エラー:', {
          type: event.type,
          target: event.target
        });
      },
    }
  );

  const sendMessage = useCallback((message: string) => {
    if (!isClient) {
      console.warn('クライアントサイドでのみメッセージを送信できます');
      return;
    }
    console.log('送信メッセージ:', message);
    wssSendMessage(message);
  }, [wssSendMessage, isClient]);

  const connectionStatus = {
    [ReadyState.CONNECTING]: '接続中...',
    [ReadyState.OPEN]: '接続済み',
    [ReadyState.CLOSING]: '切断中...',
    [ReadyState.CLOSED]: '切断',
    [ReadyState.UNINSTANTIATED]: '未接続',
  }[readyState];

  useEffect(() => {
    if (isClient) {
      console.log('WebSocket status:', connectionStatus);
    }
  }, [connectionStatus, isClient]);

  useEffect(() => {
    if (lastMessage && isClient) {
      console.log('受信メッセージ:', lastMessage);
    }
  }, [lastMessage, isClient]);

  return {
    sendMessage,
    lastMessage,
    readyState,
    connectionStatus,
  };
} 