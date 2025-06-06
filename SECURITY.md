# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Best Practices for Users

1. **API Key Safety**:
   - Never commit API keys to version control
   - Use environment variables for API keys
   - Rotate API keys regularly

2. **Dependencies**:
   - Keep dependencies updated
   - Run `cargo audit` regularly
   - Review dependency changes

3. **Data Handling**:
   - This library does not store any data
   - All API responses are passed directly to your application
   - Implement your own data validation and sanitization

Thank you for helping keep finnhub secure!