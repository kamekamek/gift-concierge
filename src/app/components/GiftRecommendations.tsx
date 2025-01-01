import React from 'react';
import { Box, Card, CardContent, Typography, Grid } from '@mui/material';

interface GiftRecommendation {
  name: string;
  price: string;
  store: string;
  reason: string;
  etiquette_advice: string;
}

interface GiftRecommendationsProps {
  recommendations: GiftRecommendation[];
}

export const GiftRecommendations: React.FC<GiftRecommendationsProps> = ({ recommendations }) => {
  return (
    <Box sx={{ maxWidth: 1200, mx: 'auto', p: 3 }}>
      <Typography variant="h5" component="h2" gutterBottom>
        おすすめのお返しギフト
      </Typography>
      
      <Grid container spacing={3}>
        {recommendations.map((gift, index) => (
          <Grid item xs={12} md={4} key={index}>
            <Card>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  {gift.name}
                </Typography>
                
                <Typography color="text.secondary" gutterBottom>
                  価格: {gift.price}
                </Typography>
                
                <Typography color="text.secondary" gutterBottom>
                  取扱店: {gift.store}
                </Typography>
                
                <Typography variant="body2" paragraph>
                  <strong>選定理由:</strong><br />
                  {gift.reason}
                </Typography>
                
                <Typography variant="body2">
                  <strong>マナーアドバイス:</strong><br />
                  {gift.etiquette_advice}
                </Typography>
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Box>
  );
}; 