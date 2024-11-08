import fast_geo_distance


def test_geodesic():
    lat_a = 52.518992275104445
    lon_a = 13.404800164623978
    lat_b = 48.140313048369265
    lon_b = 11.563939007231188

    distance = fast_geo_distance.geodesic(lat_a, lon_a, lat_b, lon_b)

    assert distance == 504347.652400197
