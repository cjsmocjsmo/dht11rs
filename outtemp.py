import requests
import json
import argparse

def get_current_temperature(latitude, longitude):
    """Fetches the current temperature for a given latitude and longitude.

    Args:
        latitude: The latitude of the location.
        longitude: The longitude of the location.

    Returns:
        The current temperature in degrees Fahrenheit, or None if an error occurs.
    """

    base_url = "https://api.weather.gov/points/"
    url = f"{base_url}{latitude},{longitude}"
    # print(url)

    try:
        response = requests.get(url)
        response.raise_for_status()  # Raise an exception for error HTTP statuses

        data = response.json()
        forecast_url = data['properties']['forecast']

        forecast_response = requests.get(forecast_url)
        forecast_response.raise_for_status()

        forecast_data = forecast_response.json()
        temperature = forecast_data['properties']['periods'][0]['temperature']

        return temperature

    except requests.exceptions.RequestException as e:
        print(f"Error fetching weather data: {e}")
        return None

def main():
    parser = argparse.ArgumentParser(description='Get the current temperature for a given latitude and longitude.')
    # parser.add_argument('latitude', type=float, help='The latitude of the location.')
    # parser.add_argument('longitude', type=float, help='The longitude of the location.')

    # args = parser.parse_args()
    latit = 47.37849
    longit = -122.94207

    temperature = get_current_temperature(latit, longit)
    # if temperature is not None:
    #     print(f"The current temperature is {temperature}°F")
    # else:
    #     print("Could not fetch the temperature.")

    return temperature

if __name__ == "__main__":
    main()