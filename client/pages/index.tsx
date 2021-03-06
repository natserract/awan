import React from "react";
import type { NextPage } from 'next'
import styles from './styles';
import { makeStyles, useTheme } from '@mui/styles'
import Typography from '@mui/material/Typography'

const useStyles = makeStyles(styles);

const Index: NextPage = () => {
  const classes = useStyles();

  return (
    <section className="home-container">
      <div className="home-content">
        <Typography variant="h3" gutterBottom>
          SaasLayerDev
          </Typography>
        <p>Objects are fundamental entities stored in Amazon S3.</p>
      </div>
    </section>
  )
}

export default Index
