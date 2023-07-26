EXPECTED_BAKE_TIME = 40
LAYER_PREP_TIME = 2


def bake_time_remaining(minutes_in_oven):
    """Computes the time remaining for baking to complete (in minutes)

    :param minutes_in_oven: int - total time lasagna has been in the oven
    :return: int - the time remaining for the lasagna to finish baking (in minutes)
    """
    return EXPECTED_BAKE_TIME - minutes_in_oven


def preparation_time_in_minutes(number_of_layers):
    """Computes the preparation time of the lasagna based on number_of_layers

    :param number_of_layers: int - the number of layers in the lasagna.
    :return: int - total preparation time (in minutes)
    """
    return LAYER_PREP_TIME * number_of_layers


def elapsed_time_in_minutes(number_of_layers, elapsed_bake_time):
    """Calculate the elapsed cooking time.

    :param number_of_layers: int - the number of layers in the lasagna.
    :param elapsed_bake_time: int - elapsed cooking time.
    :return: int - total time elapsed (in minutes) preparing and cooking.

    This function takes two integers representing the number of lasagna layers and the
    time already spent baking and calculates the total elapsed minutes spent cooking the
    lasagna.
    """
    return preparation_time_in_minutes(number_of_layers) + elapsed_bake_time

