import React, { useState } from 'react';
import { Box, Button, FormControl, InputLabel, MenuItem, Select, TextField, Typography } from '@mui/material';

interface GiftFormData {
  giftType: string;
  priceRange: string;
  relationship: string;
  eventType: string;
  additionalNotes?: string;
}

interface GiftFormProps {
  onSubmit: (data: GiftFormData) => void;
}

export const GiftForm: React.FC<GiftFormProps> = ({ onSubmit }) => {
  const [formData, setFormData] = useState<GiftFormData>({
    giftType: '',
    priceRange: '',
    relationship: '',
    eventType: '',
    additionalNotes: '',
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | { name?: string; value: unknown }>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name as string]: value,
    }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(formData);
  };

  return (
    <Box component="form" onSubmit={handleSubmit} sx={{ maxWidth: 600, mx: 'auto', p: 3 }}>
      <Typography variant="h5" component="h1" gutterBottom>
        お返しギフト提案フォーム
      </Typography>

      <FormControl fullWidth margin="normal">
        <InputLabel>受け取ったギフトの種類</InputLabel>
        <Select
          name="giftType"
          value={formData.giftType}
          onChange={handleChange}
          required
        >
          <MenuItem value="money">お金</MenuItem>
          <MenuItem value="gift">プレゼント</MenuItem>
          <MenuItem value="giftcard">ギフトカード</MenuItem>
        </Select>
      </FormControl>

      <FormControl fullWidth margin="normal">
        <InputLabel>価格帯</InputLabel>
        <Select
          name="priceRange"
          value={formData.priceRange}
          onChange={handleChange}
          required
        >
          <MenuItem value="1-3">1-3万円</MenuItem>
          <MenuItem value="3-5">3-5万円</MenuItem>
          <MenuItem value="5-10">5-10万円</MenuItem>
          <MenuItem value="10+">10万円以上</MenuItem>
        </Select>
      </FormControl>

      <FormControl fullWidth margin="normal">
        <InputLabel>贈り主との関係</InputLabel>
        <Select
          name="relationship"
          value={formData.relationship}
          onChange={handleChange}
          required
        >
          <MenuItem value="boss">上司</MenuItem>
          <MenuItem value="colleague">同僚</MenuItem>
          <MenuItem value="friend">友人</MenuItem>
          <MenuItem value="relative">親戚</MenuItem>
        </Select>
      </FormControl>

      <FormControl fullWidth margin="normal">
        <InputLabel>イベントの種類</InputLabel>
        <Select
          name="eventType"
          value={formData.eventType}
          onChange={handleChange}
          required
        >
          <MenuItem value="wedding">結婚祝い</MenuItem>
          <MenuItem value="birth">出産祝い</MenuItem>
          <MenuItem value="job">就職祝い</MenuItem>
          <MenuItem value="other">その他</MenuItem>
        </Select>
      </FormControl>

      <TextField
        fullWidth
        margin="normal"
        name="additionalNotes"
        label="その他特記事項"
        multiline
        rows={4}
        value={formData.additionalNotes}
        onChange={handleChange}
      />

      <Button
        type="submit"
        variant="contained"
        color="primary"
        fullWidth
        sx={{ mt: 3 }}
      >
        ギフトを提案する
      </Button>
    </Box>
  );
}; 